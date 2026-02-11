# css/css-flexbox/css-flexbox-row-reverse-wrap.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/css-flexbox-row-reverse-wrap.html"
}
```

## style[0]

```css

        .container {
            display: flex;
            flex-flow: row-reverse wrap;
            writing-mode: vertical-rl;
            border: 2px solid black;
            height: 90px;
        }
        .item {
            width: 15px;
            height: 45px;

            /* make sure UA that doesn't support writing mode and flexbox fails. */
            float: right;
        }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “flex-flow”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
