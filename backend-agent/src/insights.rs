use crate::types::TwitterInsight;
use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::MakeTlsConnector;
pub async fn get_insights_context() -> Result<(String, Vec<TwitterInsight>), anyhow::Error> {
    let mut builder = SslConnector::builder(SslMethod::tls()).expect("unable to create sslconnector builder");
    builder.set_verify(openssl::ssl::SslVerifyMode::NONE);
    let connector = MakeTlsConnector::new(builder.build());

    let db_config = format!(
        "host={} user={} password={} dbname={} port={} sslmode=require",
        std::env::var("DB_HOST").expect("DB_HOST must be set"),
        std::env::var("DB_USER").expect("DB_USER must be set"),
        std::env::var("DB_PASSWORD").expect("DB_PASSWORD must be set"),
        std::env::var("DB_NAME").expect("DB_NAME must be set"),
        std::env::var("DB_PORT").expect("DB_PORT must be set"),
    );

    let (client, connection) = tokio_postgres::connect(&db_config, connector)
        .await
        .expect("Failed to connect to database");

    // The connection object performs the actual communication with the database so spawn it off to run on its own
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let rows = client
        .query("SELECT * FROM twitter_defi_insights", &[])
        .await
        .expect("Failed to query database");

    // Convert rows to insights
    let x_insights: Vec<TwitterInsight> = rows
        .iter()
        .map(|row| {
            let sentiment_float: f64 = row.get("sentiment");
            TwitterInsight {
                tweet_text: row.get("tweet_text"),
                author: row.get("author"),
                timestamp: row.get("timestamp"),
                strategy_type: row.get("strategy_type"),
                protocol_mentioned: row.get("protocol_mentioned"),
                sentiment: (sentiment_float * 100.0) as i32, // Scale by 100 to preserve 2 decimal places
                engagement_score: row.get("engagement_score"),
            }
        })
        .collect();

    println!("Loaded {} insights from database", x_insights.len());

    Ok((TwitterInsight::format_insights(&x_insights), x_insights))
}

impl TwitterInsight {
    pub fn format_insights(insights: &Vec<TwitterInsight>) -> String {
        let mut formatted = String::new();
        formatted.push_str("## Latest Twitter Starknet DeFi Insights Context\n\n");

        for (i, insight) in insights.iter().enumerate() {
            formatted.push_str(&format!("### Insight {}\n", i + 1));
            formatted.push_str(&format!("**Author:** @{}\n", insight.author));
            formatted.push_str(&format!(
                "**Time:** {}\n",
                insight.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
            ));
            formatted.push_str(&format!("**Strategy:** {}\n", insight.strategy_type));
            formatted.push_str(&format!("**Protocol:** {}\n", insight.protocol_mentioned));
            formatted.push_str(&format!("**Sentiment Score:** {:.2}\n", insight.sentiment));
            formatted.push_str(&format!(
                "**Engagement Score:** {}\n",
                insight.engagement_score
            ));
            formatted.push_str(&format!("**Tweet:** {}\n\n", insight.tweet_text));
            formatted.push_str("---\n\n");
        }

        // total insights
        formatted.push_str("## Summary Statistics\n");
        formatted.push_str(&format!("Total Insights: {}\n", insights.len()));

        // average sentiment
        let avg_sentiment =
            insights.iter().map(|i| i.sentiment).sum::<i32>() / insights.len() as i32;
        formatted.push_str(&format!("Average Sentiment: {:.2}\n", avg_sentiment));

        //  total engagement
        let total_engagement: i32 = insights.iter().map(|i| i.engagement_score).sum();
        formatted.push_str(&format!("Total Engagement: {}\n", total_engagement));

        formatted
    }
}
