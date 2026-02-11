# css/css-view-transitions/view-transition-types-matches-case-sensitive-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/view-transition-types-matches-case-sensitive-ref.html"
}
```

## style[0]

```css

.test {
  width: 100px;
  height: 100px;
  background: green;
}

#container {
  display: flex;
  flex-direction: row;
  gap: 10px;
}

body { background: lightpink; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
