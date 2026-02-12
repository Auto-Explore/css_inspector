# css/css-writing-modes/orthogonal-parent-shrink-to-fit-001q.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/orthogonal-parent-shrink-to-fit-001q.html"
}
```

## style[0]

```css

.test {
    border:thin solid;
    font:20px/1 Ahem;
}
.target {
    color:blue;
    height:3em; /* height: 3em is not required. IE11 and Edge12 compute height to ICB height if
    not set. We want the test to focus exclusively on shrink-to-fit algorithm. */
    writing-mode:vertical-rl;
}
.border {
    border-right:blue solid .5em;
}
.next {
    color:orange;
}
.inline-block {
    display:inline-block;
}
.float {
    float:left;
}
h3 {
    clear:both;
}
h3.fail {
    color:red;
}
table {
    border-spacing:0px;
}
td {
    padding:0px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
