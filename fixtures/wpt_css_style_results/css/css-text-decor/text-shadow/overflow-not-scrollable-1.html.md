# css/css-text-decor/text-shadow/overflow-not-scrollable-1.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/text-shadow/overflow-not-scrollable-1.html"
}
```

## style[0]

```css

div {
  height: 200px; width: 200px; overflow: auto;
  text-shadow: 210px 210px 2px gray;
  font-size: 50px;
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
