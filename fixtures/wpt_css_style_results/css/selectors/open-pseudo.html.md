# css/selectors/open-pseudo.html

```json
{
  "format_version": 3,
  "file": "css/selectors/open-pseudo.html"
}
```

## style[0]

```css


dialog:open + #afterdialog,
details:open + #afterdetails,
select:open + #afterselect {
  will-change: text-decoration;
}

```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
