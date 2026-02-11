# css/css-ruby/ruby-layout-internal-boxes-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-ruby/ruby-layout-internal-boxes-ref.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:14px/1 monospace; padding:0; margin:0;
}

input {
  width: 20px;
  -webkit-appearance: none;
  border: 1px solid black;
  background: white;
}

.thg { display: table-header-group; }
.trg { display: table-row-group; }
.tfg { display: table-footer-group; }
.tr  { display: table-row; }
.tc  { display: table-cell; }
.tcg { display: table-column-group; }
.tco { display: table-column; }
.tca { display: table-caption; }
.t   { display: table; }

.rb  { display: ruby-base; }
.rt  { display: ruby-text; }
.rbc { display: ruby-base-container; }
.rtc { display: ruby-text-container; }

.x { background: red}
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “-webkit-appearance”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
