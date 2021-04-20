class Menutoggle extends HTMLElement {
    connectedCallback() {
      const button = this.querySelector("button:first-child");
      const target = this.getAttribute("target");
      const cb = (evt) => {
          if (target) {
              const t = document.querySelector(`[${target}]`);
              if (t) {
                  t.classList.toggle("sidebar-open")
              }
          }
      }
      if (button) {
          button.addEventListener("click", cb);
      }
    }
}

window.customElements.define('menu-toggle', Menutoggle);