# css/css-tables/crashtests/atomic-item-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-tables/crashtests/atomic-item-crash.html"
}
```

## style[0]

```css

  #target { appearance: textarea; container-type: size; offset: auto url(); }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “offset”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
