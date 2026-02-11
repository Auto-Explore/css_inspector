# css/css-flexbox/css-flexbox-row.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/css-flexbox-row.html"
}
```

## style[0]

```css

        .container {
            display: flex;
            flex-flow: row;
            writing-mode: vertical-rl;
            border: 2px solid black;
            height: 90px;
        }
        .item {
            line-height: 0;

            /* make sure UA that doesn't support writing mode and flexbox fails. */
            float: right;
        }
        .color-block {
          display: inline-block;
          width: 15px;
          height: 45px;
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
