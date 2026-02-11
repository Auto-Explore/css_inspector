# css/CSS2/margin-padding-clear/padding-applies-to-016.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/padding-applies-to-016.xht"
}
```

## style[0]

```css

   /* reset everything to be sure we don't introduce oddities */
   table { border: 0; border-spacing: 0; padding: 0; margin: 0; line-height: 1; }
   td { border: 0; padding: 0; margin: 0; line-height: 1; }
   span { border: 0; padding: 0; margin: 0; line-height: 1; display: inline; }

   /* the test */
   body { margin-top: 12em; }
   table { background: red; }
   span.control { background: green; color: white; }
   span.test { padding-top: 10em; }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
