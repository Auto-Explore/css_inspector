# css/css-contain/contain-size-select-elem-005-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-size-select-elem-005-ref.html"
}
```

## style[0]

```css

  select {
    color: transparent;
    /* We make scrollbars transparent because some <option> elements can cause
       overflow, which can cause scrollbars to be active in the test and
       inactive in the reference.  But the test only cares about the sizing. */
    scrollbar-color: transparent transparent;
  }
  .fsMedium {
    /* custom styling for some select elements, which is allowed to influence
       their size (in the same way that it influences the size of an empty
       select element): */
    font-size: 10px;
  }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
