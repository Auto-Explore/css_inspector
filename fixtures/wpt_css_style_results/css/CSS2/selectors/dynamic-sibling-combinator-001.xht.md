# css/CSS2/selectors/dynamic-sibling-combinator-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/selectors/dynamic-sibling-combinator-001.xht"
}
```

## style[0]

```css
<![CDATA[
      div { color: red; }
      [class=foo] + div { color: green; }
      [class=foo] + div + div { color: green; }
      [class=foo] + div + div + div { color: green; }
      [class=foo] + div + div + div + div { color: green; }
      [class=foo] + div + div + div + div + div { color: green; }
      [class=foo] + div + div + div + div + div + div { color: green; }
      [class=foo] + div + div + div + div + div + div + div { color: green; }
      [class=foo] + div + div + div + div + div + div + div + div { color: green; }
      [class=foo] + div + div + div + div + div + div + div + div + div { color: green; }
      [class=foo] + div + div + div + div + div + div + div + div + div + div { color: green; }
  ]]>
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid input.",
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
