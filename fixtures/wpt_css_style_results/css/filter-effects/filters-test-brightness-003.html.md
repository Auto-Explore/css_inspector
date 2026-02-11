# css/filter-effects/filters-test-brightness-003.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/filters-test-brightness-003.html"
}
```

## style[0]

```css


        div.brightness_noset {
            width: 200px;
            height: 200px;
            background-color: rgb(0, 255, 0);
            filter: brightness(0);
            filter: brightness();
        }
    
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “filter”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “filter”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
