# css/cssom/getComputedStyle-pseudo-picker-icon.html

```json
{
  "format_version": 3,
  "file": "css/cssom/getComputedStyle-pseudo-picker-icon.html"
}
```

## style[0]

```css

select,
select::picker(select) {
  appearance: base-select;
}

#test-select {
  width: 321px;
}

#test-select::picker-icon {
  width: 13px;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
