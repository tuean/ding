import { createRouter, createWebHashHistory } from "vue-router";
// import Layout from '@/layout/index.vue'
// import Home from '@/views/home/Home.vue'
import Editor from "../components/Editor.vue";
import View from '../components/View.vue'
import App from '../App.vue'

const routes = [
    {
        path: "/",
        redirect: "/app/editor"
    },
    {
        name: "app",
        path: "/app",
        component: App,
        children: [
            {
                name: 'editor',
                path: "editor",
                component: () => import("../components/Editor.vue")
              },
              {
                name: 'view',
                path: "view",
                component: () => import("../components/View.vue")
              },
              
        ]
    },
    {
      name: 'viewTop',
      path: "/viewTop",
      component: () => import("../components/ViewTop.vue")
    },
    {
      name: 'clipboard',
      path: '/clipboard',
      component: () => import("../components/clipboard/Base.vue")
    }
];

export default createRouter({
  history: createWebHashHistory(),
  routes
});
