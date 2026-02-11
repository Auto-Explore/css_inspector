# css/selectors/i18n/lang-pseudo-class-across-shadow-boundaries.html

```json
{
  "format_version": 3,
  "file": "css/selectors/i18n/lang-pseudo-class-across-shadow-boundaries.html"
}
```

## style[0]

```css

#testCases div { width: 100px; height: 25px; }
#host1:lang(zh) { background: green; }
#host3 > :lang(ja) { background: green; }
#host4 > div { background: red; }
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
 div { width: 100px; height: 25px; background: red; } div:lang(de) { background: green; } 
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
 :lang(ja)::slotted(*) { background: green !important; } 
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
