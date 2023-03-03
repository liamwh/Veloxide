# GraphQL

## Introduction to GraphQL

GraphQL is a query language for APIs that was developed by Facebook in 2012 and has since gained popularity among developers for a number of reasons:

- **Improved developer experience:** With GraphQL, developers can focus on defining the data schema and let the GraphQL engine handle the complexities of querying the data. This can lead to faster development times and a more intuitive development experience.

- **Flexibility:** Because GraphQL allows clients to specify exactly what data they need, it enables greater flexibility in how data is retrieved and displayed. It also allows for easier iteration and evolution of APIs over time.

- **Better documentation:** GraphQL APIs come with self-documentation capabilities, which means that developers can easily discover what data is available and how to query it without having to consult external documentation.

- **Strong typing:** GraphQL has a strong type system that allows for better validation of queries and can help catch errors earlier in the development process.

- **Increased efficiency:** GraphQL allows you to request only the data that you need, which reduces the amount of data transferred over the network and can result in faster response times.

Consequently, using GraphQL can lead to faster, more efficient, and more flexible API development, which can improve the developer experience and ultimately lead to better products.

## GraphQL in Velox

Velox implements a iGraphQL web interface on `http://localhost:8000` by default. This interface allows you to explore the GraphQL schema and execute queries against the API. Most commonly, GraphQL will be used for retrieving the Aggregate view models or a subset of their data. For example, you can retrieve the `BankAccountView` model and all of its data by executing the following query:

> Note: As Velox is not yet seeded at the time of writing, you will need to first create an account.

```graphql
{
  bankAccount(id: "your-bank-account-id-here") {
    accountId
    balance
    writtenChecks
    account_transactions{
      description
      amount
    }
  }
}
```

The book describing the async-graphql library used in Velox can be found [here](https://async-graphql.github.io/async-graphql/en/index.html).
