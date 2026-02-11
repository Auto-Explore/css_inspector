# css/css-typed-om/the-stylepropertymap/declared/declared.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-typed-om/the-stylepropertymap/declared/declared.tentative.html"
}
```

## style[0]

```css

div {
  height: 10px;
  width: 50%;
  width: 'lemon';
  --foo: auto;
  transition-duration: 1s, 2s;
  color: 10;
}

#target {
  height: 20px;
  --foo: 1s;
  width: 10%;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
