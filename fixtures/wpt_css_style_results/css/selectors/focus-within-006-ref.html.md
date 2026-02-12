# css/selectors/focus-within-006-ref.html

```json
{
  "format_version": 3,
  "file": "css/selectors/focus-within-006-ref.html"
}
```

## style[0]

```css

:focus {
  outline: none;

  /* Make the caret invisible
     since it matches the color of the text, which is transparent,
     while keeping the text readable thanks to its shadow.
     Not using the caret-color property as it is too new to be relied on. */
  color: transparent; text-shadow: black 0px 0px 0px;
}

div {
  border: solid 15px green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
