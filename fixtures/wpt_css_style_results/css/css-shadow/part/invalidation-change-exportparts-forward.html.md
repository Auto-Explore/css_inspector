# css/css-shadow/part/invalidation-change-exportparts-forward.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/part/invalidation-change-exportparts-forward.html"
}
```

## style[0]

```css
#c-e-outer::part(part-forwarded) { color: red; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[1]

```css
span { color: green; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
