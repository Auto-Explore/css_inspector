# css/css-flexbox/flex-flow-007.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flex-flow-007.html"
}
```

## style[0]

```css

  #test {
    background-color: red;
    display: flex;
    flex-flow: column nowrap;
    height: 100px;
    width: 100px;
  }
  #test div {
    background-color: green;
    height: 50px;
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
