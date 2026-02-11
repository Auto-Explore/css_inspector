# css/css-conditional/container-queries/container-name-tree-scoped.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/container-name-tree-scoped.html"
}
```

## style[0]

```css

        :host { container-name: foo; }
      
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css

    #container-name-host > div {
      container-type: inline-size;
    }
    #t1 { color: red; }
    @container foo (width > 0px) {
      #t1 { color: green; }
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[2]

```css

        ::slotted(div) {
          container-name: foo;
        }
      
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[3]

```css

    #container-name-slotted > div > div {
      container-type: inline-size;
    }
    #t2 { color: red; }
    @container foo (width > 0px) {
      #t2 { color: green; }
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[4]

```css

          :host { container-name: foo; }
          #t3 { color: red; }
          @container foo (width > 0px) {
            #t3 { color: green; }
          }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[5]

```css

          :host { container-name: foo; }
          ::slotted(div) { color: red; }
          @container foo (width > 0px) {
            ::slotted(div) {
              color: green;
            }
          }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
