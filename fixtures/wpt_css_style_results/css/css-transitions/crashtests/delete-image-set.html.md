# css/css-transitions/crashtests/delete-image-set.html

```json
{
  "format_version": 3,
  "file": "css/css-transitions/crashtests/delete-image-set.html"
}
```

## style[0]

```css

  div { background: url(#); }
  div { background: -webkit-image-set(url(#) 1x); }
  div { background: image-set(url(#) 1x); }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
