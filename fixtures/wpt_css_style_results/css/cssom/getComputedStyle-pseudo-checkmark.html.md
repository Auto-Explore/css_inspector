# css/cssom/getComputedStyle-pseudo-checkmark.html

```json
{
  "format_version": 3,
  "file": "css/cssom/getComputedStyle-pseudo-checkmark.html"
}
```

## style[0]

```css

select,
select::picker(select) {
  appearance: base-select;
}

#test-option {
  width: 321px;
}

#test-option::checkmark {
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
