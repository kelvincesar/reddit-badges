# Reddit badges

Develop this to run everyday and like any post in the Reddit. 

This allow to keep earning the Reddit Streak badge which I was in a great streak and didn't want to keep login everyday :)



## Executing

```sh
Usage: reddit-badges [OPTIONS] --client-id '<CLIENT_ID>' --client-secret '<CLIENT_SECRET>' --username '<USERNAME>' --password '<PASSWORD>'

Options:
  -s, --subreddit <SUBREDDIT>          The subreddit from which to fetch the first post [default: fujifilm]
      --client-id <CLIENT_ID>          Reddit client ID
      --client-secret <CLIENT_SECRET>  Reddit client secret
      --username <USERNAME>            Reddit username
      --password <PASSWORD>            Reddit password
```


## Development

You can use `nix develop` to build the project.
