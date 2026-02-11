# css/CSS2/normal-flow/block-in-inline-insert-010.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/normal-flow/block-in-inline-insert-010.xht"
}
```

## style[0]

```css

   body &gt; span { border: 3px solid blue }
   body &gt; span &gt; span { border: 3px solid cyan }
   body &gt; span &gt; span:after { content: "Ten" }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
