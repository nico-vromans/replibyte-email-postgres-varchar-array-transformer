# replibyte-email-postgres-varchar-array-transformer

This is a custom EmailTransformer for [RepliByte], enabling it on a PostgreSQL Varchar Array field (MySQL and MongoDB
untested).

In order to compile and use this transformer, you can follow along with [these instructions], but this is a slightly
updated method (primarily the target):

- `rustup target add wasm32-wasip1`
- `cargo build --release --target wasm32-wasip1`
- update your \<CONFIG\>.y(a)ml to reflect th path:
    ```yaml
    source:
      # ...
      transformers:
        - database: <DATABASE>
          table: <TABLE>
          columns:
            - name: <COLUMN_NAME>
              transformer_name: custom-wasm
              transformer_options:
                path: "path/to/transformer/target/wasm32-wasip1/release/replibyte-email-postgres-varchar-array-transformer.wasm"
    ```

[//]: # (Links)

[RepliByte]: https://www.replibyte.com/

[these instructions]: https://www.replibyte.com/docs/advanced-guides/web-assembly-transformer/
