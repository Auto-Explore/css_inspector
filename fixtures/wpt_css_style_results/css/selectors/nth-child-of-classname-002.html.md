# css/selectors/nth-child-of-classname-002.html

```json
{
  "format_version": 3,
  "file": "css/selectors/nth-child-of-classname-002.html"
}
```

## style[0]

```css

    div {
        display: block;
        width: 100px;
        height: 10px;
        background-color: red;
    }
    .container {
        height: 100px;
    }
    .reference {
        background-color: green;
    }
    div:nth-child(-n+3 of .test) { /* selects the first 3 with class test */
        background-color: green;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
