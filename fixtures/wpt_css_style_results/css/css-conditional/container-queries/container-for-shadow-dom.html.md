# css/css-conditional/container-queries/container-for-shadow-dom.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/container-for-shadow-dom.html"
}
```

## style[0]

```css

  #ancestor-across-root,
  #ancestor-skip-slotting,
  #ancestor-slotted,
  #ancestor-host,
  #ancestor-part,
  #ancestor-slotted-before,
  #ancestor-host-before,
  #ancestor-part-before,
  #ancestor-inner-part,
  #ancestor-slot-fallback,
  #inner-scope-host-part {
    width: 400px;
    container-type: inline-size;
  }
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

        @container (width = 400px) {
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

        div {
          width: 200px;
          container-type: inline-size;
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

      @container (width = 200px) {
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

        slot {
          display: block;
          width: 200px;
          container-type: inline-size;
        }
        @container (width = 200px) {
          ::slotted(#t3) { color: green; }
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

        @container (width = 400px) {
          :host(#t4) { color: green; }
        }
      
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[6]

```css

        div {
          width: 200px;
          container-type: inline-size;
        }
      
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[7]

```css

      @container (width = 200px) {
        #ancestor-part > div::part(part) { color: green; }
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[8]

```css

        slot {
          display: block;
          width: 200px;
          container-type: inline-size;
        }
        @container (width = 400px) {
          ::slotted(#t6)::before {
            content: "X";
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

## style[9]

```css

      #t6 {
        width: 400px;
        container-type: inline-size;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[10]

```css

        :host {
          width: 200px;
          container-type: inline-size;
        }
        @container (width = 200px) {
          :host(#t7)::before {
            content: "X";
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

## style[11]

```css

    @container (width = 200px) {
      #ancestor-part-before > div::part(part)::before {
        content: "X";
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

## style[12]

```css

        div {
          width: 200px;
          container-type: inline-size;
        }
      
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[13]

```css

    @container (width = 200px) {
      #ancestor-inner-part > div::part(inner-part) { color: green; }
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[14]

```css

        div {
          width: 200px;
          container-type: inline-size;
        }
      
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[15]

```css

            div {
              width: 200px;
              container-type: inline-size;
            }
          
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[16]

```css

      div {
        width: 200px;
        container-type: inline-size;
      }
      @container (width = 200px) {
        #t10 { color: green; }
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[17]

```css

        div {
          width: 200px;
          container-type: inline-size;
        }
        #t11 { color: red; }
      
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[18]

```css

      @container (width = 200px) {
        #container-for-part > div::part(part) { color: green; }
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[19]

```css

        div {
          width: 200px;
          container-type: inline-size;
        }
        @container (width = 200px) {
          :host::part(part) { color: green; }
        }
      
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[20]

```css

      @container --part2 (width >= 0px) {
        #named-part2-child { color: green; }
      }
      #named-part1 {
        container: --part1 / inline-size;
        color: red;
      }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “container”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[21]

```css

    #named-part-host::part(part2) { container: --part2 / inline-size; }
    @container --part1 (width >= 0px) {
      #named-part-host::part(part1-child) { color: green; }
    }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “container”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[22]

```css

      @container --slot-container (width >= 0px) {
        ::slotted(#slotted1) { color: green; }
      }
      #named-slotted-container {
        container: --slot-container / inline-size;
      }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “container”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[23]

```css

    @container --slot-container (width >= 0px) {
      #slotted2 { color: green; }
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[24]

```css

      :host {
        container: --host-container / inline-size;
      }
      @container --host-container (width >= 0px) {
        #host-descendant { color: green; }
        ::slotted(#host-slotted1) { color: green; }
      }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “container”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[25]

```css

    @container --host-container (width >= 0px) {
      #host-slotted2 { color: green; }
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[26]

```css

      .container {
        width: 400px;
        container-type: inline-size;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[27]

```css

    @container (width = 200px) {
      #pseudo-1::before { color: green; }
    }
    @container (width = 400px) {
      #pseudo-2::before { color: green; }
    }
    #pseudo-1 {
      container-type: inline-size;
      width: 200px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
