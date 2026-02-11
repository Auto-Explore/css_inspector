# css/selectors/invalidation/has-with-pseudo-class.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/has-with-pseudo-class.html"
}
```

## style[0]

```css

main:has(input) div { color: grey }
main:has(#checkbox:checked) > #subject { color: red }
main:has(#option:checked) > #subject { color: red }
main:has(#checkbox:disabled) > #subject { color: green }
main:has(#option:disabled) > :is(#subject, #subject2) { color: green }
main:has(#optgroup:disabled) > #subject { color: blue }
main:not(:has(#checkbox:enabled)) > #subject3 { color: green }
main:not(:has(#option:enabled)) :is(#subject3, #subject4) { color: green }
main:not(:has(#optgroup:enabled)) > #subject3 { color: blue }
main:has(#text_input:valid) > #subject { color: yellow }
main:not(:has(#text_input:invalid)) > #subject2 { color: yellow }
main:has(#form:valid) > #subject3 { color: yellow }
main:not(:has(#form:invalid)) > #subject4 { color: yellow }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
