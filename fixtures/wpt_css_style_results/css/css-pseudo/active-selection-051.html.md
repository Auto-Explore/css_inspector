# css/css-pseudo/active-selection-051.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/active-selection-051.html"
}
```

## style[0]

```css

  div
    {
      color: transparent;
      font-size: 300%;
    }

  div::selection
    {
      foo: bar;
    }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “foo”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
