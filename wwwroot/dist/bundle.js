(()=>{"use strict";const e="http://0.0.0.0:8080";let t=!1,n=50;function o(e){n=parseInt(e);const t=document.getElementById("w-size");t&&(t.textContent=e)}function a(){return n}function d(e){const t=document.getElementById("w-seed");t&&(t.textContent=e)}function s(e){const t=document.getElementById("w-robot");t&&(t.textContent=e)}function r(){return t}var c,l,i,m,g;let u,w;!function(e){e.Start="Start",e.Stop="Stop",e.Pause="Pause",e.Resume="Resume"}(c||(c={})),function(e){e.Sunny="Sunny",e.Rainy="Rainy",e.Foggy="Foggy",e.TropicalMonsoon="TropicalMonsoon",e.TrentinoSnow="TrentinoSnow"}(l||(l={})),function(e){e.Morning="Morning",e.Afternoon="Afternoon",e.Night="Night"}(i||(i={})),function(e){e.DeepWater="DeepWater",e.ShallowWater="ShallowWater",e.Sand="Sand",e.Grass="Grass",e.Street="Street",e.Hill="Hill",e.Mountain="Mountain",e.Snow="Snow",e.Lava="Lava",e.Teleport="Teleport",e.Wall="Wall"}(m||(m={})),function(e){e.Rock="Rock",e.Tree="Tree",e.Garbage="Garbage",e.Fire="Fire",e.Coin="Coin",e.Bin="Bin",e.Crate="Crate",e.Bank="Bank",e.Water="Water",e.Market="Market",e.Fish="Fish",e.Building="Building",e.Bush="Bush",e.JollyBlock="JollyBlock",e.Scarecrow="Scarecrow",e.None="None"}(g||(g={}));const y=new Image;y.src="dist/tiles/bin.webp";const I=new Image;I.src="dist/tiles/chest.webp";const b=new Image;b.src="dist/tiles/cobblestone.webp";const f=new Image;f.src="dist/tiles/coin.svg";const h=new Image;h.src="dist/tiles/crate.webp";const p=new Image;p.src="dist/tiles/deep_water.webp";const E=new Image;E.src="dist/tiles/dirt.webp";const B=new Image;B.src="dist/tiles/fire.svg";const k=new Image;k.src="dist/tiles/fish.webp";const S=new Image;S.src="dist/tiles/grass.webp";const v=new Image;v.src="dist/tiles/lava.webp";const C=new Image;C.src="dist/tiles/market.webp";const x=new Image;x.src="dist/tiles/portal.webp";const $=new Image;$.src="dist/tiles/sand.webp";const T=new Image;T.src="dist/tiles/snow.webp";const L=new Image;L.src="dist/tiles/street.webp";const W=new Image;W.src="dist/tiles/trash.png";const F=new Image;F.src="dist/tiles/tree.webp";const R=new Image;R.src="dist/tiles/wall.webp";const M=new Image;M.src="dist/tiles/water.webp";const N=new Image;N.src="dist/tiles/stone.webp";const z=new Image;z.src="dist/tiles/bush.webp";const _=new Image;_.src="dist/tiles/scarecrow.svg";const D=new Image;D.src="dist/tiles/lucky.webp";const O=new Image;function P(){const e=document.getElementById("draw-area");u=Math.min(window.innerWidth-400,window.innerHeight-20),w=Math.floor(u/a()),u=w*a(),e&&(e.width=u,e.height=u)}O.src="dist/tiles/creeper.jpg";const j=new WebSocket(`${e.replace("http","ws")}/commands`);let A,J=!1;function G(e){const t={command:e};j.readyState===WebSocket.OPEN?j.send(JSON.stringify(t)):alert(`Command socket is not open. ${j.readyState}`)}function H(){A=new WebSocket(`${e.replace("http","ws")}/updates`),j.onclose=()=>{console.log("Disconnected from command socket")},j.onerror=e=>{alert(`WebSocket error:${e}`)},A.onopen=()=>{console.log("connected to updates socket ")},A.onclose=e=>{console.log("Disconnected from update socket",e)},A.onmessage=e=>{try{const n=JSON.parse(e.data);!function(e){const t=document.getElementById("coordinates");t&&(t.textContent=`X: ${e.row} Y: ${e.col}`)}(n.robot_data.coordinate),function(e){const t=document.getElementById("energy");t&&(t.textContent=e.toString())}(n.robot_data.energy_level),function(e){const t=document.getElementById("stone-number");t&&null!=e&&(t.textContent=e.toString(),function(e){const t=document.getElementById("stone");t&&(t.style.display=e?"flex":"none")}(0!==e))}((t=n.robot_data.backpack).Rock),function(e){const t=document.getElementById("wood-number");t&&null!=e&&(t.textContent=e.toString(),function(e){const t=document.getElementById("wood");t&&(t.style.display=e?"flex":"none")}(0!==e))}(t.Tree),function(e){const t=document.getElementById("trash-number");t&&null!=e&&(t.textContent=e.toString(),function(e){const t=document.getElementById("trash");t&&(t.style.display=e?"flex":"none")}(0!==e))}(t.Garbage),function(e){const t=document.getElementById("fire-number");t&&null!=e&&(t.textContent=e.toString(),function(e){const t=document.getElementById("fire");t&&(t.style.display=e?"flex":"none")}(0!==e))}(t.Fire),function(e){const t=document.getElementById("coin-number");t&&null!=e&&(t.textContent=e.toString(),function(e){const t=document.getElementById("coin");t&&(t.style.display=e?"flex":"none")}(0!==e))}(t.Coin),function(e){const t=document.getElementById("water-number");t&&null!=e&&(t.textContent=e.toString(),function(e){const t=document.getElementById("water");t&&(t.style.display=e?"flex":"none")}(0!==e))}(t.Water),function(e){const t=document.getElementById("fish-number");t&&null!=e&&(t.textContent=e.toString(),function(e){const t=document.getElementById("fish");t&&(t.style.display=e?"flex":"none")}(0!==e))}(t.Fish),function(e){const t=document.getElementById("bush-number");t&&null!=e&&(t.textContent=e.toString(),function(e){const t=document.getElementById("bush");t&&(t.style.display=e?"flex":"none")}(0!==e))}(t.Bush),function(e){const t=document.getElementById("jolly-number");t&&null!=e&&(t.textContent=e.toString(),function(e){const t=document.getElementById("jolly");t&&(t.style.display=e?"flex":"none")}(0!==e))}(t.JollyBlock),function(e){let t=document.getElementById("time");t&&(t.textContent=e.time),t=document.getElementById("day-time"),t&&(t.textContent=e.day_time.toString()+", "+e.weather.toString())}(n.environment),function(e){const t=document.getElementById("weather");if(!t)return void console.error('Element with id "weather" not found');const n="dist/img/";if(e.weather!=l.Sunny)switch(e.weather){case l.Rainy:t.src=`${n}rain.png`;break;case l.Foggy:t.src=`${n}fog.png`;break;case l.TropicalMonsoon:t.src=`${n}storm.png`;break;case l.TrentinoSnow:t.src=`${n}snow.png`;break;default:t.src=""}else switch(e.day_time){case i.Morning:t.src=`${n}morning.png`;break;case i.Afternoon:t.src=`${n}afternoon.png`;break;case i.Night:t.src=`${n}night.png`}}(n.environment),function(e,t){P(),o(e.length.toString());const n=document.getElementById("draw-area"),a=n.getContext("2d");if(a){a.clearRect(0,0,n.width,n.height);for(let t=0;t<e.length;t++)for(let n=0;n<e.length;n++){const o=e[t][n];if(o){const e=n*w,d=t*w;switch(o.tile_type){case m.DeepWater:a.drawImage(p,d,e,w,w);break;case m.ShallowWater:a.drawImage(M,d,e,w,w);break;case m.Sand:a.drawImage($,d,e,w,w);break;case m.Grass:a.drawImage(S,d,e,w,w);break;case m.Street:a.drawImage(L,d,e,w,w);break;case m.Hill:a.drawImage(E,d,e,w,w);break;case m.Mountain:a.drawImage(N,d,e,w,w);break;case m.Snow:a.drawImage(T,d,e,w,w);break;case m.Lava:a.drawImage(v,d,e,w,w);break;case m.Teleport:a.drawImage(x,d,e,w,w);break;case m.Wall:a.drawImage(R,d,e,w,w)}switch(o.content){case g.Rock:a.drawImage(b,d,e,w,w);break;case g.Tree:a.drawImage(F,d,e,w,w);break;case g.Garbage:a.drawImage(W,d,e,w,w);break;case g.Fire:a.drawImage(B,d,e,w,w);break;case g.Coin:a.drawImage(f,d,e,w,w);break;case g.Bin:a.drawImage(y,d,e,w,w);break;case g.Crate:a.drawImage(h,d,e,w,w);break;case g.Bank:a.drawImage(I,d,e,w,w);break;case g.Market:a.drawImage(C,d,e,w,w);break;case g.Fish:a.drawImage(k,d,e,w,w);break;case g.Bush:a.drawImage(z,d,e,w,w);break;case g.Scarecrow:a.drawImage(_,d,e,w,w);break;case g.JollyBlock:a.drawImage(D,d,e,w,w);case g.Building:case g.Water:case g.None:}}}a.drawImage(O,t.row*w,t.col*w,w,w),a.strokeStyle="#f00",a.lineWidth=4,a.strokeRect(t.row*w,t.col*w,w,w)}}(n.map,n.robot_data.coordinate),function(e){const t=document.getElementById("log-box");for(;t.children.length>11;)t.removeChild(t.firstChild);const n=document.createElement("div");n.className="log-entry",n.textContent=e,t.appendChild(n),t.scrollTop=t.scrollHeight}(n.event),J&&(G(c.Stop),U(),new Promise((e=>setTimeout(e,4e3))).then(),alert(`${function(){const e=document.getElementById("w-robot");return e?e.textContent:""}()} terminated his job, press ok to start over`),window.location.reload()),J="Terminated"===n.event}catch(t){console.error(e.data),alert(`Error deserializing update:${t}`)}var t}}function U(){A.close(),j.close()}function X(){new Promise((e=>setTimeout(e,500))).then((()=>{G(c.Start),H()}))}document.getElementById("reset").addEventListener("click",(()=>{G(c.Stop),U(),window.location.reload()})),window.addEventListener("load",(async()=>{try{const t=await fetch(`${e}/robots`);if(!t.ok)throw new Error("Failed to fetch robots from the server");const n=(await t.json()).robots;if(!Array.isArray(n))throw new Error("Response data is not an array");const o=document.getElementById("robot");n.forEach((e=>{const t=document.createElement("option");t.value=e,t.text=e,o.add(t)}))}catch(e){console.error(e),alert("An error occurred while fetching robots from the server")}document.getElementById("generate-seed").addEventListener("click",(async function(){try{const t=await fetch(`${e}/randomSeed`);if(!t.ok)throw new Error("Failed to fetch random seed from the server");const n=await t.json();document.getElementById("seed").value=n.seed.toString()}catch(e){console.error(e),alert("An error occurred while fetching random seed from the server")}}));const t=document.getElementById("show-advanced"),n=document.getElementById("advanced-options"),a=document.getElementById("show-advanced-title");t&&n&&a&&t.addEventListener("click",(()=>{""==n.style.display||"none"==n.style.display?(n.style.display="block",a.textContent="Hide"):(n.style.display="none",a.textContent="Show advanced")}));const r=document.getElementById("upload-world"),c=document.getElementById("download-world"),l=document.getElementById("start-button-title"),i=()=>{r.files&&r.files.length>0?(l.textContent="Upload & start",document.getElementById("world-size").disabled=!0,document.getElementById("seed").disabled=!0,document.getElementById("download-world").disabled=!0,document.getElementById("generate-seed").disabled=!0,document.getElementById("wait").disabled=!1,document.getElementById("robot").disabled=!1):c.checked?(l.textContent="Download",document.getElementById("robot").disabled=!0,document.getElementById("wait").disabled=!0,document.getElementById("upload-world").disabled=!0):(l.textContent="Start",document.getElementById("world-size").disabled=!1,document.getElementById("seed").disabled=!1,document.getElementById("generate-seed").disabled=!1,document.getElementById("wait").disabled=!1,document.getElementById("robot").disabled=!1)};c.addEventListener("change",i),r.addEventListener("change",i),c.addEventListener("change",(()=>{const e=c.checked;document.getElementById("robot").disabled=e,document.getElementById("wait").disabled=e,document.getElementById("generate-seed").disabled=!1,document.getElementById("start-button-title").textContent=e?"Download":r.files&&r.files.length>0?"Upload and start":"Start",document.getElementById("upload-world").disabled=e})),document.getElementById("generate-form").addEventListener("submit",(async function(t){t.preventDefault();const n=document.getElementById("world-size").value,a=document.getElementById("seed").value,l=document.getElementById("wait").value,i=document.getElementById("robot").value,m=c.checked,g=r.files&&r.files.length>0,u={worldSize:parseInt(n),seed:parseInt(a),wait:parseInt(l),robot:i};try{const t=document.getElementById("loading");if(t.style.display="flex",r.files&&r.files.length>0){const t=r.files[0],n=new FormData;if(n.append("world",t),n.append("robot",i),n.append("wait",l),!(await fetch(`${e}/uploadWorld`,{method:"POST",body:n})).ok)throw new Error("Failed to upload file to the server");X(),d("?"),s(i);const o=document.getElementById("modal");o&&(o.style.display="none")}else if(m){if(!(await fetch(`${e}/downloadWorld`,{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({worldSize:u.worldSize,seed:u.seed})})).ok)throw new Error("Failed to send data to the server");const t=`${e}/worlds/cobblestone_world.zst`;try{const e=await fetch(t);if(!e.ok)throw new Error(`Failed to download file: ${t}`);const n=await e.blob(),o="cobblestone_world.zst",a=document.createElement("a");a.href=URL.createObjectURL(n),a.download=o,a.click()}catch(e){console.error("Error downloading file:",e),alert("Failed to download the file.")}}else{if(!(await fetch(`${e}/generate`,{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify(u)})).ok)throw new Error("Failed to send data to the server");const t=document.getElementById("modal");t&&(t.style.display="none")}t.style.display="none",m||g||(o(n),d(a),s(i),X())}catch(e){alert(`An error occurred while sending data to the server\n${e}`),document.getElementById("loading").style.display="none"}}))})),window.addEventListener("resize",P),window.addEventListener("load",P),document.getElementById("pause").addEventListener("click",(()=>{var e;e=!r(),t=e;const n=document.getElementById("pause"),o=document.getElementById("pause-title");r()&&o&&n?(o.textContent="Resume",G(c.Pause)):(o.textContent="Pause",G(c.Resume))})),document.addEventListener("DOMContentLoaded",(function(){const e=document.getElementById("log-box"),t=document.getElementById("toggle-log"),n=document.getElementById("log-title");t.addEventListener("click",(function(){"none"===e.style.display||""===e.style.display?(e.style.display="block",n.textContent="Hide Logs"):(n.textContent="Show Logs",e.style.display="none")}))}))})();
//# sourceMappingURL=bundle.js.map