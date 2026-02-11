# css/css-shadow/part/simple.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/part/simple.html"
}
```

## style[0]

```css
#c-e::part(partp) { color: green; }
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
span { color: red; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
