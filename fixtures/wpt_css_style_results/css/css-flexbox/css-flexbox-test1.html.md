# css/css-flexbox/css-flexbox-test1.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/css-flexbox-test1.html"
}
```

## style[0]

```css

        .container {
            display: flex;
            flex-flow: row;
            writing-mode: vertical-rl;
            color: white;
        }
        .item {
            background: green;
            height: 3em;
            width: 3em;

            /* make sure UA that doesn't support writing mode and flexbox fails. */
            float: right;
        }
        .error {
            position: absolute;
            background: red;
            height: 9em;
            width: 3em;
            z-index: -1;
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
