# css/css-counter-styles/counter-style-at-rule/shadow-dom-part.html

```json
{
  "format_version": 3,
  "file": "css/css-counter-styles/counter-style-at-rule/shadow-dom-part.html"
}
```

## style[0]

```css

@counter-style foo {
  system: fixed;
  symbols: A B C;
}
#host::part(list) {
  list-style-type: foo;
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

@counter-style foo {
  system: fixed;
  symbols: D E F;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
