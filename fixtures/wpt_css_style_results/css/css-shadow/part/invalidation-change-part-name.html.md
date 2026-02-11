# css/css-shadow/part/invalidation-change-part-name.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/part/invalidation-change-part-name.html"
}
```

## style[0]

```css
#c-e::part(partp) { color: red; }
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
