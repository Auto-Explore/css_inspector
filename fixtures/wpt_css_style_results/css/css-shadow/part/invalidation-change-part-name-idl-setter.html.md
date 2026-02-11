# css/css-shadow/part/invalidation-change-part-name-idl-setter.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/part/invalidation-change-part-name-idl-setter.html"
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
#c-e::part(new-partp) { color: green; }
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

## style[2]

```css
span { color: blue; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
