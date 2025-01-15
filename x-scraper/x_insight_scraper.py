import os
from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.common.keys import Keys
import openai
import psycopg2
import time
import json
from openai import OpenAI
from dotenv import load_dotenv
import sys

class StarknetDefiScraper:
    def __init__(self, db_config, openai_key):
        self.username = os.getenv("TWITTER_USERNAME")
        self.password = os.getenv("TWITTER_PASSWORD")
        self.search_queries = [
            "starknet defi strategy",
            "starknet reward",
            "starknet yields", 
            "starknet defi protocols",
            "starknet liquidity",
            "starknet farming",
            "starknet yield guide",
            "starknet defi guide"
        ]
        self.openai = OpenAI(api_key=openai_key.strip())
        self.db_config = db_config
        try:
            self.conn = psycopg2.connect(**self.db_config)
            self.cur = self.conn.cursor()
        except psycopg2.Error as e:
            print(f"Database connection failed: {e}")
            raise e
        
    def process_tweet_with_llm(self, tweet_text):
        try:
            prompt = """
            Analyze this tweet about Starknet DeFi and extract:
            1. Strategy type (yield farming, lending, liquidity provision, etc)
            2. Protocol mentioned
            3. Sentiment score (-100 to 100)

            Tweet: {tweet}

            You must respond with valid JSON in this exact format:
            {{"strategy_type": "type", "protocol": "name", "sentiment": number}}
            """.format(tweet=tweet_text)

            response = self.openai.chat.completions.create(
                model="gpt-4o-mini",
                messages=[{
                    "role": "system",
                    "content": "You are a helpful assistant that always responds with valid JSON"
                },
                {
                    "role": "user", 
                    "content": prompt
                }],
                temperature=0.7
            )

            try:
                content = response.choices[0].message.content
                return json.loads(content)
            except json.JSONDecodeError as e:
                print(f"Failed to parse JSON response: {content}")
                # Return default values if parsing fails
                return {
                    "strategy_type": "unknown",
                    "protocol": "unknown",
                    "sentiment": 0.0
                }

        except openai.APIError as e:
            print(f"OpenAI API error: {e}")
            return {
                "strategy_type": "error",
                "protocol": "error",
                "sentiment": 0.0
            }


    def save_to_db(self, tweet_data, llm_analysis):
        try:
            query = """
            INSERT INTO twitter_defi_insights 
            (tweet_text, author, timestamp, strategy_type, protocol_mentioned, 
             sentiment, engagement_score)
            VALUES (%s, %s, %s, %s, %s, %s, %s)
            """

            engagement_score = (
                tweet_data['likes'] + 
                tweet_data['retweets'] * 2 + 
                tweet_data['replies'] * 3
            )

            self.cur.execute(query, (
                tweet_data['text'],
                tweet_data['author'],
                tweet_data['timestamp'],
                llm_analysis['strategy_type'],
                llm_analysis['protocol'],
                llm_analysis['sentiment'],
                engagement_score
            ))

            self.conn.commit()
            print("Tweet saved to database successfully")
        except psycopg2.Error as e:
            print(f"Database error: {e}")
            # Attempt to reconnect
            try:
                self.conn = psycopg2.connect(**self.db_config)
                self.cur = self.conn.cursor()
            except psycopg2.Error as e:
                print(f"Failed to reconnect to database: {e}")


        
    def login(self, browser):
        print("Logging in to Twitter...")
        browser.get("https://twitter.com/i/flow/login")
        time.sleep(3)

        try:
            # Input username
            username_input = browser.find_element(By.CSS_SELECTOR, 'input[autocomplete="username"]')
            username_input.send_keys(self.username)
            username_input.send_keys(Keys.RETURN)
            time.sleep(2)

            # Input password
            password_input = browser.find_element(By.CSS_SELECTOR, 'input[type="password"]')
            password_input.send_keys(self.password)
            password_input.send_keys(Keys.RETURN)
            time.sleep(5)

            print("Login successful")
        except Exception as e:
            print(f"Login failed: {e}")
            browser.quit()
            raise e


    def scrape_tweets(self):
        
        browser = webdriver.Chrome()

        try:         
            print("Starting tweet scraping...")

            self.login(browser)

            for query in self.search_queries:
                print(f"Processing query: {query}")
                url = f"https://twitter.com/search?q={query}&src=typed_query&f=live"
                browser.get(url)
                time.sleep(5)  # Let the page load

                # Scroll to load more tweets
                for _ in range(5):
                    browser.execute_script("window.scrollTo(0, document.body.scrollHeight);")
                    time.sleep(2)

                tweets = browser.find_elements(By.CSS_SELECTOR, 'article[data-testid="tweet"]')
                print(f"Found {len(tweets)} tweets")

                for tweet in tweets:
                    try:
                        tweet_data = {
                            'text': tweet.find_element(By.CSS_SELECTOR, '[data-testid="tweetText"]').text,
                            'author': tweet.find_element(By.CSS_SELECTOR, '[data-testid="User-Name"]').text,
                            'timestamp': tweet.find_element(By.TAG_NAME, 'time').get_attribute('datetime'),
                            'likes': int(tweet.find_element(By.CSS_SELECTOR, '[data-testid="like"]').text or 0),
                            'retweets': int(tweet.find_element(By.CSS_SELECTOR, '[data-testid="retweet"]').text or 0),
                            'replies': int(tweet.find_element(By.CSS_SELECTOR, '[data-testid="reply"]').text or 0)
                        }

                        print(f"Processing tweet: {tweet_data['text'][:50]}...")
                        llm_analysis = self.process_tweet_with_llm(tweet_data['text'])
                        self.save_to_db(tweet_data, llm_analysis)
                        print("Tweet processed and saved")

                    except Exception as e:
                        print(f"Error processing tweet: {e}")
                        continue
        finally:
            browser.quit()
            if hasattr(self, 'cur') and self.cur:
                self.cur.close()
            if hasattr(self, 'conn') and self.conn:
                self.conn.close()

        

# Usage
if __name__ == "__main__":
    load_dotenv()

    db_config = {
        "dbname": os.getenv("DB_NAME"),
        "user": os.getenv("DB_USER"),
        "password": os.getenv("DB_PASSWORD"),
        "host": os.getenv("DB_HOST"),
        "port": os.getenv("DB_PORT")
    }

    try:
        conn = psycopg2.connect(**db_config)
        print("Database connection successful")
        conn.close()
    except psycopg2.OperationalError as e:
        print(f"Failed to connect to database: {e}")
        sys.exit(1)

    openai_key = os.getenv("OPENAI_API_KEY", "").strip()
    if not openai_key:
        print("OpenAI API key not found")
        sys.exit(1)

    try:
        scraper = StarknetDefiScraper(
            db_config=db_config,
            openai_key=openai_key
        )
        scraper.scrape_tweets()
    except Exception as e:
        print(f"Error during scraping: {e}")
    finally:
        if hasattr(scraper, 'cur') and scraper.cur:
            scraper.cur.close()
        if hasattr(scraper, 'conn') and scraper.conn:
            scraper.conn.close()

