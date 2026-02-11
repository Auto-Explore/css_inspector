# css/css-shadow/part/precedence-part-vs-part.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/part/precedence-part-vs-part.html"
}
```

## style[0]

```css
#c-e-outer::part(part-forwarded) { color: green; }
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
span { color: blue; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[2]

```css
#c-e-inner::part(partp) { color: red; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
