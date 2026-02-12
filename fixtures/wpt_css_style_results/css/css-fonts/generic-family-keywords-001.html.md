# css/css-fonts/generic-family-keywords-001.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/generic-family-keywords-001.html"
}
```

## style[0]

```css

div {
  font-size: 10px;
}
#ahem {
  font-family: Ahem;
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

@font-face {
  font-family: ${keyword};
  src: local(Ahem), url('/fonts/Ahem.ttf');
}
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

@font-face {
  font-family: "${keyword}";
  src: local(Ahem), url('/fonts/Ahem.ttf');
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
