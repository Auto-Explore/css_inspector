# css/css-text-decor/text-shadow/overflow-not-scrollable-2.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/text-shadow/overflow-not-scrollable-2.html"
}
```

## style[0]

```css

div {
  height: 200px; width: 200px; overflow: auto;
  text-shadow: 100px 100px 30px black;
  font-size: 50px;
  font-weight: bold;
}

/* work around potential font overflow */
span { margin-left: 5px }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “text-shadow”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
