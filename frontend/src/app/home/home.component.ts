import { Component } from '@angular/core';
import { NewPostComponent } from '../new-post/new-post.component';

@Component({
  selector: 'app-home',
  standalone: true,
  imports: [NewPostComponent],
  templateUrl: './home.component.html',
  styleUrl: './home.component.scss'
})
export class HomeComponent {

}
