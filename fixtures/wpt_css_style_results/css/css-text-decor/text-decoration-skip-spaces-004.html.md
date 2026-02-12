# css/css-text-decor/text-decoration-skip-spaces-004.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/text-decoration-skip-spaces-004.html"
}
```

## style[0]

```css

div {
    /* Use separate longhands
       as Safari does not support the full syntax of the text-decoration shorthand
       at the time of writing,
       but that's not what we're testing here.
     */
    text-decoration: underline;
    text-decoration-color: blue;

    text-decoration-skip-spaces: start end; /* Theoretically not needed, as that's the default behavior per L3 */
    color: orange;
    font-size: 2em;
    white-space: break-spaces;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
