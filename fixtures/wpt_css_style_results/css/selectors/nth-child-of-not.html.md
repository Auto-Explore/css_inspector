# css/selectors/nth-child-of-not.html

```json
{
  "format_version": 3,
  "file": "css/selectors/nth-child-of-not.html"
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
    .reference, .fallback {
        background-color: green;
    }
    div:nth-child(even of :not(.reference)) { /* selects even divs with class test */
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
