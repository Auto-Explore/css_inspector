# css/css-tables/rules-groups.html

```json
{
  "format_version": 3,
  "file": "css/css-tables/rules-groups.html"
}
```

## style[0]

```css

table { margin: 1em 1em 2em 1em; }

/* Modify style of the inter-group borders, assuming default UA rules are in effect. */

/* With border-block-start suppressed, the block-end border should still appear: */
#a > * { border-block-start: none; }

/* Change the color of the border: */
#b > * { border-block-end-color: blue; }

/* Change the thickness of the border: */
#c > * { border-block-end-width: 5px; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
