# css/css-ruby/ruby-intrinsic-isize-003-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-ruby/ruby-intrinsic-isize-003-ref.html"
}
```

## style[0]

```css

  .ruby {
    border-left: 5px solid blue;
    border-right: 10px solid blue;
    padding: 0 20px 0 10px;
    margin: 0 10px 0 20px;
  }
  div {
    display: inline-block;
    border: 1px solid black;
    font-kerning: none; /* disable kerning, because in the reference file
                            it might occur across <span> boundaries */
  }
  .ruby > span {
    white-space: nowrap;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
