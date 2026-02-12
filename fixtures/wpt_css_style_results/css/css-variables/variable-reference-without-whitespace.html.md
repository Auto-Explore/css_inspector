# css/css-variables/variable-reference-without-whitespace.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-reference-without-whitespace.html"
}
```

## style[0]

```css

p {
    counter-reset: -- --a -a;
    --dash:-;
}

#test_1 span::before {
    counter-increment: var(--dash)-;
    content: counter(--);
}
#test_2 span::before {
    counter-increment: var(--dash)-a;
    content: counter(--a);
}
#test_3 span::before {
    counter-increment: var(--dash)a;
    content: counter(-a);
}

#control_1 span::before {
    counter-increment: --;
    content: counter(--);
}
#control_2 span::before {
    counter-increment: --a;
    content: counter(--a);
}
#control_3 span::before {
    counter-increment: -a;
    content: counter(-a);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
