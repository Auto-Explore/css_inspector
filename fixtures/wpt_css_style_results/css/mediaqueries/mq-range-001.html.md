# css/mediaqueries/mq-range-001.html

```json
{
  "format_version": 3,
  "file": "css/mediaqueries/mq-range-001.html"
}
```

## style[0]

```css

div {
    width: 100px;
    height: 100px;
    background-color: red;
}

/* if the syntax is accepted, whether (width 500px) is true or not doesn't matter,
   as the second part of the "or" clause will be true,
   but if the syntax is rejected, then the whole thing is ignored.
*/
@media (width 500px) or (min-width: 0) {
    div { background-color: green; }
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
