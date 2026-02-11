# css/compositing/root-element-filter-background-clip-text-crash.html

```json
{
  "format_version": 3,
  "file": "css/compositing/root-element-filter-background-clip-text-crash.html"
}
```

## style[0]

```css

* {
  filter: sepia(1);
  background-color: rgb(179, 31, 172);
  background-clip: text;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “filter”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
