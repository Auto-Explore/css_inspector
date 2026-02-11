# css/css-contain/content-visibility/content-visibility-028.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-028.html"
}
```

## style[0]

```css

.hidden {
  content-visibility: hidden;
}
.child {
  width: 50px;
  height: 50px;
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

## style[1]

```css

    .container {
      width: 150px;
      height: 150px;
      background: lightblue;
    }
    
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
