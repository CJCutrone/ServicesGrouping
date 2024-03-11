# Services Grouping
Auto grouping tool to interact with Planning Center Services API. Initial iterations of the project will work without direct interaction with the API, but will be iterated upon to include API integration. The CLI will store it's own copy of the user data and group data. This data can be updated at any time via an update command.

```bash
services-grouping update "datasource.json"
```
Note: Accepted data source types are JSON/Excel

The main functionality of the CLI (grouping) requires a cron timer, and an end date. It uses data that has previously been loaded into the database via the update command.

```bash
services-grouping group "0 0 * * 0,3" "1/28/1990"
```
(Example command call that groups every Sunday and Wednesday at midnight, until January 28th, 1990)