# css/selectors/nth-child-of-nth-child.html

```json
{
  "format_version": 3,
  "file": "css/selectors/nth-child-of-nth-child.html"
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
    div:nth-child(even of :nth-child(even)) { /* selects every other of every other */
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
