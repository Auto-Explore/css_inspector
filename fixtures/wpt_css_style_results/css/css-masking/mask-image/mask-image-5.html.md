# css/css-masking/mask-image/mask-image-5.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-image-5.html"
}
```

## style[0]

```css

      div {
        background-color: purple;
        width: 50px;
        height: 50px;
      }
      div.mask-by-data-url {
        /* a 50x50 opaque blue rect */
        mask: url("data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIj8+Cjxzdmcgd2lkdGg9IjUwIiBoZWlnaHQ9IjUwIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPgogIDxyZWN0IHg9IjAiIHk9IjAiICB3aWR0aD0iNTAiIGhlaWdodD0iNTAiIGZpbGw9ImJsdWUiIGZpbGwtb3BhY2l0eT0iMSIvPgo8L3N2Zz4K");
      }
    
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “mask”.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
