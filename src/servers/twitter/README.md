# Twitter/X Server

**Version:** 0.1.0  
**Total Tools:** 5

<details>
<summary><strong>GetMentions</strong></summary>

**Description:** Fetches mentions to the user from Twitter. Required API and ACCESS values can be valued as 'null' if the info is not available.

**Parameters:**
- `count`: The max count of tweets to be fetched
- `latest_id`: The Tweet ID to fetch mentions after

</details>

<details>
<summary><strong>GetTimeline</strong></summary>

**Description:** Fetches the user's timeline from Twitter. Required API and ACCESS values can be valued as 'null' if the info is not available.

**Parameters:**
- `count`: The max count of tweets to be fetched
- `latest_id`: The Tweet ID to fetch tweets after

</details>

<details>
<summary><strong>PostTweet</strong></summary>

**Description:** Post a tweet to Twitter. Required API and ACCESS values can be valued as 'null' if the info is not available.

**Parameters:**
- `tweet`: Text to post on Twitter

</details>

<details>
<summary><strong>ReplyToTweet</strong></summary>

**Description:** Reply a tweet to Twitter. Required API and ACCESS values can be valued as 'null' if the info is not available.

**Parameters:**
- `reply`: Text for Twitter reply
- `reply_to_tweet_id`: Tweet ID to reply to

</details>

<details>
<summary><strong>SearchTweets</strong></summary>

**Description:** Search tweets from Twitter. Required API and ACCESS values can be valued as 'null' if the info is not available.

**Parameters:**
- `query`: Search query for Twitter search
- `count`: The max count of tweets to be fetched
- `sort_order`: The Twitter sort method used for the search

</details> 