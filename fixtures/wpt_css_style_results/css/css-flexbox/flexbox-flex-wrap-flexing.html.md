# css/css-flexbox/flexbox-flex-wrap-flexing.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-flex-wrap-flexing.html"
}
```

## style[0]

```css

        .container {
            display: flex;
            width: 150px;
            height: 100px;
            flex-wrap: wrap;
            background: red;
        }
        .item {
            min-width: 100px;
            flex: 1;
            height: 50px;
            display: inline-block; /*to fail the test if display:flex fails*/;
            background: green;
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
