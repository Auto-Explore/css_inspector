# css/css-shadow/part/exportparts-different-scope.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/part/exportparts-different-scope.html"
}
```

## style[0]

```css

my-foo::part(text) { color: green; }
my-bar::part(text) { color: red; background-color: red; }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
